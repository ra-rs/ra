#[doc = "Register `ECCPRCR2` reader"]
pub struct R(crate::R<ECCPRCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCPRCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCPRCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCPRCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCPRCR2` writer"]
pub struct W(crate::W<ECCPRCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCPRCR2_SPEC>;
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
impl From<crate::W<ECCPRCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCPRCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCPRCR2` reader - Register Write Control"]
pub type ECCPRCR2_R = crate::BitReader<ECCPRCR2_A>;
#[doc = "Register Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCPRCR2_A {
    #[doc = "0: Disable writes to the protected registers"]
    _0 = 0,
    #[doc = "1: Enable writes to the protected registers."]
    _1 = 1,
}
impl From<ECCPRCR2_A> for bool {
    #[inline(always)]
    fn from(variant: ECCPRCR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCPRCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCPRCR2_A {
        match self.bits {
            false => ECCPRCR2_A::_0,
            true => ECCPRCR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCPRCR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCPRCR2_A::_1
    }
}
#[doc = "Field `ECCPRCR2` writer - Register Write Control"]
pub type ECCPRCR2_W<'a, const O: u8> = crate::BitWriter<'a, u8, ECCPRCR2_SPEC, ECCPRCR2_A, O>;
impl<'a, const O: u8> ECCPRCR2_W<'a, O> {
    #[doc = "Disable writes to the protected registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECCPRCR2_A::_0)
    }
    #[doc = "Enable writes to the protected registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECCPRCR2_A::_1)
    }
}
#[doc = "Write Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KW2_AW {
    #[doc = "120: These bits enable or disable writes to the ECCPRCR2 bit.."]
    _1111000 = 120,
}
impl From<KW2_AW> for u8 {
    #[inline(always)]
    fn from(variant: KW2_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KW2` writer - Write Key Code"]
pub type KW2_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ECCPRCR2_SPEC, u8, KW2_AW, 7, O>;
impl<'a, const O: u8> KW2_W<'a, O> {
    #[doc = "These bits enable or disable writes to the ECCPRCR2 bit.."]
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut W {
        self.variant(KW2_AW::_1111000)
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn eccprcr2(&self) -> ECCPRCR2_R {
        ECCPRCR2_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn eccprcr2(&mut self) -> ECCPRCR2_W<0> {
        ECCPRCR2_W::new(self)
    }
    #[doc = "Bits 1:7 - Write Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn kw2(&mut self) -> KW2_W<1> {
        KW2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Protection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccprcr2](index.html) module"]
pub struct ECCPRCR2_SPEC;
impl crate::RegisterSpec for ECCPRCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eccprcr2::R](R) reader structure"]
impl crate::Readable for ECCPRCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccprcr2::W](W) writer structure"]
impl crate::Writable for ECCPRCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCPRCR2 to value 0"]
impl crate::Resettable for ECCPRCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
