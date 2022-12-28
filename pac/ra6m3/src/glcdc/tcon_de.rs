#[doc = "Register `TCON_DE` reader"]
pub struct R(crate::R<TCON_DE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCON_DE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCON_DE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCON_DE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCON_DE` writer"]
pub struct W(crate::W<TCON_DE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCON_DE_SPEC>;
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
impl From<crate::W<TCON_DE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCON_DE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV` reader - DE signal polarity inversion control."]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "DE signal polarity inversion control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "1: Inverted"]
    _1 = 1,
    #[doc = "0: Not inverted"]
    _0 = 0,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            true => INV_A::_1,
            false => INV_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
}
#[doc = "Field `INV` writer - DE signal polarity inversion control."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_DE_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_A::_1)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - DE signal polarity inversion control."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DE signal polarity inversion control."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<0> {
        INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Data Enable Polarity Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcon_de](index.html) module"]
pub struct TCON_DE_SPEC;
impl crate::RegisterSpec for TCON_DE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcon_de::R](R) reader structure"]
impl crate::Readable for TCON_DE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcon_de::W](W) writer structure"]
impl crate::Writable for TCON_DE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCON_DE to value 0"]
impl crate::Resettable for TCON_DE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
