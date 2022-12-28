#[doc = "Register `CKOCR` reader"]
pub struct R(crate::R<CKOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKOCR` writer"]
pub struct W(crate::W<CKOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKOCR_SPEC>;
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
impl From<crate::W<CKOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKOSEL` reader - Clock out source select"]
pub type CKOSEL_R = crate::FieldReader<u8, CKOSEL_A>;
#[doc = "Clock out source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKOSEL_A {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: MOSC"]
    _011 = 3,
    #[doc = "4: SOSC"]
    _100 = 4,
}
impl From<CKOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOSEL_A) -> Self {
        variant as _
    }
}
impl CKOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKOSEL_A> {
        match self.bits {
            0 => Some(CKOSEL_A::_000),
            1 => Some(CKOSEL_A::_001),
            2 => Some(CKOSEL_A::_010),
            3 => Some(CKOSEL_A::_011),
            4 => Some(CKOSEL_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKOSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKOSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKOSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKOSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKOSEL_A::_100
    }
}
#[doc = "Field `CKOSEL` writer - Clock out source select"]
pub type CKOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CKOCR_SPEC, u8, CKOSEL_A, 3, O>;
impl<'a, const O: u8> CKOSEL_W<'a, O> {
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CKOSEL_A::_000)
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CKOSEL_A::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CKOSEL_A::_010)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CKOSEL_A::_011)
    }
    #[doc = "SOSC"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CKOSEL_A::_100)
    }
}
#[doc = "Field `CKODIV` reader - Clock out input frequency Division Select"]
pub type CKODIV_R = crate::FieldReader<u8, CKODIV_A>;
#[doc = "Clock out input frequency Division Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKODIV_A {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: /128"]
    _111 = 7,
}
impl From<CKODIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKODIV_A) -> Self {
        variant as _
    }
}
impl CKODIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKODIV_A {
        match self.bits {
            0 => CKODIV_A::_000,
            1 => CKODIV_A::_001,
            2 => CKODIV_A::_010,
            3 => CKODIV_A::_011,
            4 => CKODIV_A::_100,
            5 => CKODIV_A::_101,
            6 => CKODIV_A::_110,
            7 => CKODIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKODIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKODIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKODIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKODIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKODIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKODIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CKODIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CKODIV_A::_111
    }
}
#[doc = "Field `CKODIV` writer - Clock out input frequency Division Select"]
pub type CKODIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CKOCR_SPEC, u8, CKODIV_A, 3, O>;
impl<'a, const O: u8> CKODIV_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CKODIV_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CKODIV_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CKODIV_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CKODIV_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CKODIV_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CKODIV_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CKODIV_A::_110)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CKODIV_A::_111)
    }
}
#[doc = "Field `CKOEN` reader - Clock out enable"]
pub type CKOEN_R = crate::BitReader<CKOEN_A>;
#[doc = "Clock out enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKOEN_A {
    #[doc = "0: Disable clock out"]
    _0 = 0,
    #[doc = "1: Enable clock out"]
    _1 = 1,
}
impl From<CKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CKOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKOEN_A {
        match self.bits {
            false => CKOEN_A::_0,
            true => CKOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKOEN_A::_1
    }
}
#[doc = "Field `CKOEN` writer - Clock out enable"]
pub type CKOEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CKOCR_SPEC, CKOEN_A, O>;
impl<'a, const O: u8> CKOEN_W<'a, O> {
    #[doc = "Disable clock out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CKOEN_A::_0)
    }
    #[doc = "Enable clock out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CKOEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock out source select"]
    #[inline(always)]
    pub fn ckosel(&self) -> CKOSEL_R {
        CKOSEL_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Clock out input frequency Division Select"]
    #[inline(always)]
    pub fn ckodiv(&self) -> CKODIV_R {
        CKODIV_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Clock out enable"]
    #[inline(always)]
    pub fn ckoen(&self) -> CKOEN_R {
        CKOEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock out source select"]
    #[inline(always)]
    #[must_use]
    pub fn ckosel(&mut self) -> CKOSEL_W<0> {
        CKOSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Clock out input frequency Division Select"]
    #[inline(always)]
    #[must_use]
    pub fn ckodiv(&mut self) -> CKODIV_W<4> {
        CKODIV_W::new(self)
    }
    #[doc = "Bit 7 - Clock out enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckoen(&mut self) -> CKOEN_W<7> {
        CKOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Out Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckocr](index.html) module"]
pub struct CKOCR_SPEC;
impl crate::RegisterSpec for CKOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ckocr::R](R) reader structure"]
impl crate::Readable for CKOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckocr::W](W) writer structure"]
impl crate::Writable for CKOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKOCR to value 0"]
impl crate::Resettable for CKOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
