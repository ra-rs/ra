#[doc = "Register `TDR` reader"]
pub struct R(crate::R<TDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDR` writer"]
pub struct W(crate::W<TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDR_SPEC>;
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
impl From<crate::W<TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDAT` reader - Serial transmit data"]
pub type TDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDAT` writer - Serial transmit data"]
pub type TDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDR_SPEC, u16, u16, 9, O>;
#[doc = "Field `MPBT` reader - Multi-processor transfer bit flag"]
pub type MPBT_R = crate::BitReader<MPBT_A>;
#[doc = "Multi-processor transfer bit flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPBT_A {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<MPBT_A> for bool {
    #[inline(always)]
    fn from(variant: MPBT_A) -> Self {
        variant as u8 != 0
    }
}
impl MPBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPBT_A {
        match self.bits {
            false => MPBT_A::_0,
            true => MPBT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPBT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPBT_A::_1
    }
}
#[doc = "Field `MPBT` writer - Multi-processor transfer bit flag"]
pub type MPBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TDR_SPEC, MPBT_A, O>;
impl<'a, const O: u8> MPBT_W<'a, O> {
    #[doc = "Data transmission cycles"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPBT_A::_0)
    }
    #[doc = "ID transmission cycles"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPBT_A::_1)
    }
}
#[doc = "Field `TSYNC` reader - Transmit SYNC data"]
pub type TSYNC_R = crate::BitReader<TSYNC_A>;
#[doc = "Transmit SYNC data\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSYNC_A {
    #[doc = "0: The Start Bit is transmitted as DATA SYNC."]
    _0 = 0,
    #[doc = "1: The Start Bit is transmitted as COMMAND SYNC."]
    _1 = 1,
}
impl From<TSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl TSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSYNC_A {
        match self.bits {
            false => TSYNC_A::_0,
            true => TSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSYNC_A::_1
    }
}
#[doc = "Field `TSYNC` writer - Transmit SYNC data"]
pub type TSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TDR_SPEC, TSYNC_A, O>;
impl<'a, const O: u8> TSYNC_W<'a, O> {
    #[doc = "The Start Bit is transmitted as DATA SYNC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSYNC_A::_0)
    }
    #[doc = "The Start Bit is transmitted as COMMAND SYNC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSYNC_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Serial transmit data"]
    #[inline(always)]
    pub fn tdat(&self) -> TDAT_R {
        TDAT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Multi-processor transfer bit flag"]
    #[inline(always)]
    pub fn mpbt(&self) -> MPBT_R {
        MPBT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit SYNC data"]
    #[inline(always)]
    pub fn tsync(&self) -> TSYNC_R {
        TSYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Serial transmit data"]
    #[inline(always)]
    #[must_use]
    pub fn tdat(&mut self) -> TDAT_W<0> {
        TDAT_W::new(self)
    }
    #[doc = "Bit 9 - Multi-processor transfer bit flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpbt(&mut self) -> MPBT_W<9> {
        MPBT_W::new(self)
    }
    #[doc = "Bit 12 - Transmit SYNC data"]
    #[inline(always)]
    #[must_use]
    pub fn tsync(&mut self) -> TSYNC_W<12> {
        TSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](index.html) module"]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdr::R](R) reader structure"]
impl crate::Readable for TDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdr::W](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDR to value 0xffff_ffff"]
impl crate::Resettable for TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
