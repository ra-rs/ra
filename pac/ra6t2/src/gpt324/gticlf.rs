#[doc = "Register `GTICLF` reader"]
pub struct R(crate::R<GTICLF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTICLF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTICLF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTICLF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTICLF` writer"]
pub struct W(crate::W<GTICLF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTICLF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTICLF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTICLF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICLFA` reader - GTIOCnA Output Logical Operation Function Select"]
pub type ICLFA_R = crate::FieldReader<u8, ICLFA_A>;
#[doc = "GTIOCnA Output Logical Operation Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLFA_A {
    #[doc = "0: A (no delay)"]
    _000 = 0,
    #[doc = "1: NOT A (no delay)"]
    _001 = 1,
    #[doc = "2: C (1GTCLK delay)"]
    _010 = 2,
    #[doc = "3: NOT C (1GTCLK delay)"]
    _011 = 3,
    #[doc = "4: A AND C (1GTCLK delay)"]
    _100 = 4,
    #[doc = "5: A OR C (1GTCLK delay)"]
    _101 = 5,
    #[doc = "6: A EXOR C (1GTCLK delay)"]
    _110 = 6,
    #[doc = "7: A NOR C (1GTCLK delay)"]
    _111 = 7,
}
impl From<ICLFA_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLFA_A) -> Self {
        variant as _
    }
}
impl ICLFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLFA_A {
        match self.bits {
            0 => ICLFA_A::_000,
            1 => ICLFA_A::_001,
            2 => ICLFA_A::_010,
            3 => ICLFA_A::_011,
            4 => ICLFA_A::_100,
            5 => ICLFA_A::_101,
            6 => ICLFA_A::_110,
            7 => ICLFA_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ICLFA_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ICLFA_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ICLFA_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ICLFA_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ICLFA_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ICLFA_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ICLFA_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == ICLFA_A::_111
    }
}
#[doc = "Field `ICLFA` writer - GTIOCnA Output Logical Operation Function Select"]
pub type ICLFA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTICLF_SPEC, u8, ICLFA_A, 3, O>;
impl<'a, const O: u8> ICLFA_W<'a, O> {
    #[doc = "A (no delay)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ICLFA_A::_000)
    }
    #[doc = "NOT A (no delay)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ICLFA_A::_001)
    }
    #[doc = "C (1GTCLK delay)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ICLFA_A::_010)
    }
    #[doc = "NOT C (1GTCLK delay)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ICLFA_A::_011)
    }
    #[doc = "A AND C (1GTCLK delay)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ICLFA_A::_100)
    }
    #[doc = "A OR C (1GTCLK delay)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ICLFA_A::_101)
    }
    #[doc = "A EXOR C (1GTCLK delay)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ICLFA_A::_110)
    }
    #[doc = "A NOR C (1GTCLK delay)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(ICLFA_A::_111)
    }
}
#[doc = "Field `ICLFSELC` reader - Inter Channel Signal C Select"]
pub type ICLFSELC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICLFSELC` writer - Inter Channel Signal C Select"]
pub type ICLFSELC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTICLF_SPEC, u8, u8, 6, O>;
#[doc = "Field `ICLFB` reader - GTIOCnB Output Logical Operation Function Select"]
pub type ICLFB_R = crate::FieldReader<u8, ICLFB_A>;
#[doc = "GTIOCnB Output Logical Operation Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLFB_A {
    #[doc = "0: B (no delay)"]
    _000 = 0,
    #[doc = "1: NOT B (no delay)"]
    _001 = 1,
    #[doc = "2: D (1GTCLK delay)"]
    _010 = 2,
    #[doc = "3: NOT D (1GTCLK delay)"]
    _011 = 3,
    #[doc = "4: B AND D (1GTCLK delay)"]
    _100 = 4,
    #[doc = "5: B OR D (1GTCLK delay)"]
    _101 = 5,
    #[doc = "6: B EXOR D (1GTCLK delay)"]
    _110 = 6,
    #[doc = "7: B NOR D (1GTCLK delay)"]
    _111 = 7,
}
impl From<ICLFB_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLFB_A) -> Self {
        variant as _
    }
}
impl ICLFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLFB_A {
        match self.bits {
            0 => ICLFB_A::_000,
            1 => ICLFB_A::_001,
            2 => ICLFB_A::_010,
            3 => ICLFB_A::_011,
            4 => ICLFB_A::_100,
            5 => ICLFB_A::_101,
            6 => ICLFB_A::_110,
            7 => ICLFB_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ICLFB_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ICLFB_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ICLFB_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ICLFB_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ICLFB_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ICLFB_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ICLFB_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == ICLFB_A::_111
    }
}
#[doc = "Field `ICLFB` writer - GTIOCnB Output Logical Operation Function Select"]
pub type ICLFB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTICLF_SPEC, u8, ICLFB_A, 3, O>;
impl<'a, const O: u8> ICLFB_W<'a, O> {
    #[doc = "B (no delay)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ICLFB_A::_000)
    }
    #[doc = "NOT B (no delay)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ICLFB_A::_001)
    }
    #[doc = "D (1GTCLK delay)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ICLFB_A::_010)
    }
    #[doc = "NOT D (1GTCLK delay)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ICLFB_A::_011)
    }
    #[doc = "B AND D (1GTCLK delay)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ICLFB_A::_100)
    }
    #[doc = "B OR D (1GTCLK delay)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ICLFB_A::_101)
    }
    #[doc = "B EXOR D (1GTCLK delay)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ICLFB_A::_110)
    }
    #[doc = "B NOR D (1GTCLK delay)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(ICLFB_A::_111)
    }
}
#[doc = "Field `ICLFSELD` reader - Inter Channel Signal D Select"]
pub type ICLFSELD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICLFSELD` writer - Inter Channel Signal D Select"]
pub type ICLFSELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTICLF_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:2 - GTIOCnA Output Logical Operation Function Select"]
    #[inline(always)]
    pub fn iclfa(&self) -> ICLFA_R {
        ICLFA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:9 - Inter Channel Signal C Select"]
    #[inline(always)]
    pub fn iclfselc(&self) -> ICLFSELC_R {
        ICLFSELC_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - GTIOCnB Output Logical Operation Function Select"]
    #[inline(always)]
    pub fn iclfb(&self) -> ICLFB_R {
        ICLFB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:25 - Inter Channel Signal D Select"]
    #[inline(always)]
    pub fn iclfseld(&self) -> ICLFSELD_R {
        ICLFSELD_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GTIOCnA Output Logical Operation Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfa(&mut self) -> ICLFA_W<0> {
        ICLFA_W::new(self)
    }
    #[doc = "Bits 4:9 - Inter Channel Signal C Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfselc(&mut self) -> ICLFSELC_W<4> {
        ICLFSELC_W::new(self)
    }
    #[doc = "Bits 16:18 - GTIOCnB Output Logical Operation Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfb(&mut self) -> ICLFB_W<16> {
        ICLFB_W::new(self)
    }
    #[doc = "Bits 20:25 - Inter Channel Signal D Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclfseld(&mut self) -> ICLFSELD_W<20> {
        ICLFSELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Inter Channel Logical Operation Function Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gticlf](index.html) module"]
pub struct GTICLF_SPEC;
impl crate::RegisterSpec for GTICLF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gticlf::R](R) reader structure"]
impl crate::Readable for GTICLF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gticlf::W](W) writer structure"]
impl crate::Writable for GTICLF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTICLF to value 0"]
impl crate::Resettable for GTICLF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
