#[doc = "Register `RDAYAR` reader"]
pub struct R(crate::R<RDAYAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDAYAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDAYAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDAYAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDAYAR` writer"]
pub struct W(crate::W<RDAYAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDAYAR_SPEC>;
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
impl From<crate::W<RDAYAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDAYAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATE1` reader - 1 Day Value for the ones place of days"]
pub type DATE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATE1` writer - 1 Day Value for the ones place of days"]
pub type DATE1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RDAYAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATE10` reader - 10 Days Value for the tens place of days"]
pub type DATE10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATE10` writer - 10 Days Value for the tens place of days"]
pub type DATE10_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RDAYAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENB` reader - Compare enable"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: The register value is not compared with the RDAYCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RDAYCNT counter value."]
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
#[doc = "Field `ENB` writer - Compare enable"]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u8, RDAYAR_SPEC, ENB_A, O>;
impl<'a, const O: u8> ENB_W<'a, O> {
    #[doc = "The register value is not compared with the RDAYCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENB_A::_0)
    }
    #[doc = "The register value is compared with the RDAYCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1 Day Value for the ones place of days"]
    #[inline(always)]
    pub fn date1(&self) -> DATE1_R {
        DATE1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10 Days Value for the tens place of days"]
    #[inline(always)]
    pub fn date10(&self) -> DATE10_R {
        DATE10_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Day Value for the ones place of days"]
    #[inline(always)]
    #[must_use]
    pub fn date1(&mut self) -> DATE1_W<0> {
        DATE1_W::new(self)
    }
    #[doc = "Bits 4:5 - 10 Days Value for the tens place of days"]
    #[inline(always)]
    #[must_use]
    pub fn date10(&mut self) -> DATE10_W<4> {
        DATE10_W::new(self)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<7> {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Date Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdayar](index.html) module"]
pub struct RDAYAR_SPEC;
impl crate::RegisterSpec for RDAYAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rdayar::R](R) reader structure"]
impl crate::Readable for RDAYAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdayar::W](W) writer structure"]
impl crate::Writable for RDAYAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDAYAR to value 0"]
impl crate::Resettable for RDAYAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
